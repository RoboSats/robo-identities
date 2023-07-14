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


def create_image_arrays(directory):
    image_arrays = []
    for root, _, files in os.walk(directory):
        png_files = [f for f in files if f.endswith(".png")]
        if png_files:
            array = "[\n"
            for png_file in png_files:
                png_path = os.path.join(root, png_file)
                base64_string = convert_to_webp_base64(png_path)
                array += f'    "{base64_string}",\n'
            array += "]"
            image_arrays.append((root, array))
    return image_arrays


def write_image_arrays(image_arrays, output_file):
    with open(output_file, "w") as f:
        f.write("use std::borrow::Cow;\n")
        for root, array in image_arrays:
            array_name = root.replace('/', '_').replace('#', '_')
            f.write("\n")
            f.write(f"pub static {array_name}: &[&str] = {array};\n")

if __name__ == "__main__":
    directory = "sets/set1/green"
    output_file = "src/robot_parts.rs"
    image_arrays = create_image_arrays(directory)
    write_image_arrays(image_arrays, output_file)

    directory = "backgrounds"
    output_file = "src/backgrounds.rs"
    image_arrays = create_image_arrays(directory)
    write_image_arrays(image_arrays, output_file)
