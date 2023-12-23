from flask import Flask

app = Flask(__name__)

@app.route("/")
def home():
    return "F*ck you!"

from enum import Enum

class Person(Enum):
    STRING = "string"
    INT = "int"

print(Person)
print(Person.STRING)
print(Person.INT)

# if __name__ == "__main__":
#     app.run(debug=True)
