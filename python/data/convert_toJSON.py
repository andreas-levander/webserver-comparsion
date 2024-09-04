import csv
import json

# Read the CSV file
csv_file_path = './dummy_data.csv'
json_file_path = './dummy_data_usernames.json'

data = []
with open(csv_file_path, mode='r') as file:
    csv_reader = csv.DictReader(file)
    for row in csv_reader:
        data.append(row['Username'])

# Write the data to a JSON file
with open(json_file_path, 'w') as json_file:
    json.dump(data, json_file, indent=4)

json_file_path
