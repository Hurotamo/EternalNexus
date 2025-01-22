nameSymbol(0x2261C28, "g_textClient") # Constant used in drawing text.

nameFunc(0x54EE00, "drawString", return_type="u32", args=[
    ("text_client", "f32"), 
    ("x", "u32"), 
    ("y", "u32"), 
    ("color", "u8[4]")  # Combined RGB and alpha values
])

nameFunc(0x412B50, "playAnimation", return_type="u32", args=[("anim_id","u32")])

nameFunc(0x4508F0, "renderEffect", return_type="f32", args=[
    ("effect_id", "u32"), 
    ("scene_id", "u32"), 
    ("position", "f32[3]")  # Combined x, y, and z coordinates
])