import csv
from dataclasses import dataclass
import json
import random
import lorem
import mysql.connector as mysql
import sys

db = mysql.connect(host="localhost", user="root", password="root", database="week10")

TABLE_NAME = "product"


@dataclass
class Product:
    name: str = "Default name"
    description: str = lorem.paragraph()
    manufacturer: str = "Cereal Co."
    quantity: int = random.randint(1, 1000)
    price: float = random.random() * 20
    image_url: str = None
    attributes: dict = None


def insert_into_db(product: Product):
    with db.cursor() as cur:
        cur.execute(
            f"""INSERT INTO `{TABLE_NAME}`
                (
                    name,
                    description,
                    manufacturer,
                    quantity,
                    price,
                    image_url,
                    attributes
                )
               VALUES
                (
                    %s,
                    %s,
                    %s,
                    %s,
                    %s,
                    %s,
                    %s
                )
            """,
            (
                product.name,
                product.description,
                product.manufacturer,
                product.quantity,
                product.price,
                product.image_url,
                json.dumps(product.attributes),
            ),
        )
    db.commit()
    print(f"Inserted product: {product.name}")


def process_row(row: dict[str, str]):
    print(f"Processing row: {row}")

    # Check if row is a 'type row'
    KNOWN_TYPES_LIST = ["Float", "Int", "String"]
    is_type_row = any(value in KNOWN_TYPES_LIST for value in row.values())
    if is_type_row:
        print("Row was type row. Ignoring...")
        return

    product = Product()
    extra_attributes = dict()
    for key, value in row.items():
        match key.lower():
            case "name":
                product.name = value
            case _:
                extra_attributes[key] = value
    product.attributes = extra_attributes

    if not product.image_url:
        product.image_url = f"/static/img/cereal/{product.name}"

    insert_into_db(product)


def read_csv_and_process(filename: str):
    try:
        with open(filename, mode="r", newline="", encoding="utf-8") as csvfile:
            reader = csv.DictReader(csvfile, delimiter=";")
            for row in reader:
                process_row(row)
    except FileNotFoundError:
        print(f"Error: File '{filename}' not found.")
    except Exception as e:
        print(f"An error occurred: {e}")


if __name__ == "__main__":
    csv_filename = sys.argv[1]
    read_csv_and_process(csv_filename)
