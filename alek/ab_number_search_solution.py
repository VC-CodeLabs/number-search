import re

def find_numbers(input_file_path):
    output_file_path = "C:/Users/AlekBabich/Projects/number-search/output.txt"
    
    #Open input file regardless of line endings, write output file with only Linux line endings
    with open(input_file_path, 'r') as input_file, open(output_file_path, 'w', newline='\n') as output_file:
        for line in input_file:
            numbers = re.findall(r'\d+', line)
            
            #Each line is guarenteed to have at least one number so no need to check if a line contains a number prior to accessing.
            first_number = numbers[0]
            last_number = numbers[-1]
            two_digit_number = first_number[0] + last_number[-1]
            output_file.write(two_digit_number + '\n')

    return output_file

input_file_path = "C:/Users/AlekBabich/Projects/number-search/input.txt"
print(find_numbers(input_file_path))
