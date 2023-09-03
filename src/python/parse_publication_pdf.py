import tabula
import re

def parse_publication_pdf(input_file, output_file):
    print(f"extracting data from {input_file} to {output_file}...")

    tabula.convert_into(
        input_file, # Input file
        output_file, # Output file
        output_format="tsv", 
        pages="all",
        area=[4.2, 6.5, 97, 100], # [ TOP, LEFT, BOTTOM, RIGHT ]
        relative_area=True,
        stream=True,
        guess=False
    )

    file_contents = ""

    # Read the file
    with open(output_file, "r") as file_reader:
        file_contents = file_reader.read()

    # Fix the whitespace
    for a, b in [
        (r'""', ''),
        (r'[ \t]+', r'\t'),
        (r'\t+\n', r'\n'),
        (r'\n\t+', '\n')
    ]: file_contents = re.sub(a, b, file_contents)

    # Write back to the file
    with open(output_file, "w") as file_writer:
        file_writer.write(file_contents)

    print("done.")