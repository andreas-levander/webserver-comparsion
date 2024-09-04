# app.py
from flask import Flask, jsonify, request
import csv

data = []

# Load dummy data from csv file
with open("./data/dummy_data.csv", "r", newline='') as csvfile:
    reader = csv.DictReader(csvfile)
    for row in reader:
        data.append({
            "username": row["Username"],
            "name": row['Name'],
            "address": row["Address"],
            "phone": row['Phone Number'],
            "email": row["Email"]
        })

print("csv loaded")
#print(data)

app = Flask(__name__)

@app.route("/ping")
def greet():
    return jsonify({'message': 'pong'})


@app.route("/user")
def getUserData():
    username = request.args.get('username', '')
    return jsonify(getUser(username))


def getUser(username):
    for u in data:
        if u['username'] == username:
            return u
    
    return {"username": "not found"}
#if __name__ == '__main__':
#	app.run(host='0.0.0.0', port=8090)