# Refactors the code for the ps0198 client.
# @author Cups
# @category Teos.Refactor
from ghidra.program.model.symbol import SourceType
from ghidra.app.cmd.function import ApplyFunctionSignatureCmd
from ghidra.program.model.listing import FunctionSignature
from ghidra.program.model.data import FunctionDefinitionDataType, ParameterDefinition, ParameterDefinitionImpl, ByteDataType, BooleanDataType, ShortDataType, IntegerDataType, Pointer32DataType, FloatDataType, StructureDataType, VoidDataType
from collections import OrderedDict

# Helper function to get a Ghidra Address type
def get_address(offset):
    return currentProgram.getAddressFactory().getDefaultAddressSpace().getAddress(offset)

# Helper function to either name, or create a symbol
def name_symbol(addr, name):
    var = getSymbolAt(get_address(addr))
    if var is None:
        createLabel(get_address(addr), name, False)
    else:
        var.setName(name, SourceType.USER_DEFINED)

# Creates a data type from a string.
def create_data_type(name):
    data_types = {
        "u8": ByteDataType(),
        "bool": BooleanDataType(),
        "u16": ShortDataType(),
        "u32": IntegerDataType(),
        "f32": FloatDataType(),
        "string": Pointer32DataType(),
        "ptr": Pointer32DataType(),
        "void": VoidDataType()
    }
    return data_types.get(name, StructureDataType(name, 2))

# Helper function to either name or create a function symbol, and refactor it's signature.
def name_func(addr, name, return_type, args=None):
    if args is None:
        args = []
    var = getSymbolAt(get_address(addr))
    if var is None:
        createLabel(get_address(addr), name, False)
    else:
        var.setName(name, SourceType.USER_DEFINED)

    f = FunctionDefinitionDataType(name)
    f.setReturnType(create_data_type(return_type))

    params = []
    for param, type in args:
        params.append(ParameterDefinitionImpl(param, create_data_type(type), None))
    f.setArguments(params)

    cmd = ApplyFunctionSignatureCmd(get_address(addr), f, SourceType.USER_DEFINED)
    cmd.applyTo(currentProgram)

scripts = [
    "config.py",
    "debug.py",
    "net.py",
    "player.py",
    "render.py",
    "sah.py",
    "skills.py",
    "util.py",
    "weather.py",
    "window.py"
]

for script in scripts:
    runScript(script)