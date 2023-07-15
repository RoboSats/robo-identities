import os, io, base64
from PIL import Image

QUALITY = 80
METHOD = 6  # Slowest compression method, best compression ratio and image quality

def convert_to_webp_base64(file_path: str) -> str:
    with open(file_path, "rb") as image_file:
        image_bytes = image_file.read()
    with io.BytesIO() as buffer:
        image = Image.open(io.BytesIO(image_bytes))
        image.save(buffer, "webp", quality=QUALITY, method=METHOD)
        encoded_string = base64.b64encode(buffer.getvalue())
    return encoded_string.decode("utf-8")

def sort_image_arrays_by_stacking_order(tuples_list:list)->list:
    try:
        sorted_tuples = sorted(tuples_list, key=lambda tup: int(''.join([i for i in tup[0].split('#')[1] if i.isnumeric()])))
    except: 
        # backgrounds do not have sorting number
        sorted_tuples = tuples_list
    return sorted_tuples

def create_image_arrays(directory:str)->list((str,str,int)):
    image_arrays = []
    max_length = 0
    for root, _, files in os.walk(directory):
        png_files = [f for f in files if f.endswith(".png")]
        if png_files:
            max_length = max(max_length, len(png_files))

    for root, _, files in os.walk(directory):
        png_files = [f for f in files if f.endswith(".png")]
        if png_files:
            array = "[\n"
            for i, png_file in enumerate(png_files):
                png_path = os.path.join(root, png_file)
                base64_string = convert_to_webp_base64(png_path)
                array += f'    "{base64_string}",\n'
            if i < max_length:
                array += '    PADDING,\n'*(max_length-i-1)
            array += "]"
            image_arrays.append((root, array, len(png_files)))
    image_arrays = sort_image_arrays_by_stacking_order(image_arrays)
    return image_arrays, max_length

def get_alphabetic_substring(string:str)-> str:
    for i in range(len(string)-1, -1, -1):
        if string[i].isdigit():
            result = string[i+1:]
            if result == "":
                return "BACKGROUND"
            return result
    return ""

def part_name(root:str) -> str:
    name = root.replace('/', '_').replace('#', '_').upper()
    name = get_alphabetic_substring(name)
    return name

def create_vectors(image_arrays: list, length) -> str:
    content = f'pub static PARTS: &[[&str;{length}]] = &['
    for root, _, _ in image_arrays:
        content += f' {part_name(root)},'
    content += '];\n\n'

    content += f'pub static PARTS_LENGTH: [u8; {len(image_arrays)}] = ['
    for root, _, l in image_arrays:
        content += f'{l},'
    content += '];\n\n'

    return content

def write_image_arrays(image_arrays:list, length, output_file:str):
    content = create_vectors(image_arrays,  length)
    content += 'const PADDING: &str = "";\n'
    for root, array, _ in image_arrays:
        content += "\n"
        content += f"const {part_name(root)}: [&str;{length}] =\n{array};\n"
    with open(output_file, "w") as f:
        f.write(content)

if __name__ == "__main__":
    directory = "sets/set1/green"
    output_file = "src/robot_parts.rs"
    image_arrays, length = create_image_arrays(directory)
    write_image_arrays(image_arrays, length, output_file)

    ### Regenerating backgrounds.rs requires a bit of manual editing 
    # directory = "backgrounds"
    # output_file = "src/backgrounds.rs"
    # image_arrays, length = create_image_arrays(directory)
    # write_image_arrays(image_arrays, length, output_file)
