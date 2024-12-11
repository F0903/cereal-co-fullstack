import csv
from dataclasses import dataclass
import json
import random
from typing import Self
import urllib.parse
import lorem
import mysql.connector as mysql
import sys
import urllib

db = mysql.connect(host="localhost", user="root", password="root", database="week10")

TABLE_NAME = "product"


@dataclass
class Product:
    name: str
    description: str
    manufacturer: str
    quantity: int
    price: float
    image_url: str
    attributes: dict

    @classmethod
    def semi_random(cls) -> Self:
        return cls(
            name="Default name",
            description=lorem.paragraph(),
            manufacturer="Cereal Co.",
            quantity=random.randint(1, 1000),
            price=random.random() * 20,
            image_url=None,
            attributes=None,
        )


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

    product = Product.semi_random()
    extra_attributes = dict()
    for key, value in row.items():
        match key.lower():
            case "name":
                product.name = value
            case _:
                extra_attributes[key] = value
    product.attributes = extra_attributes

    if not product.image_url:
        normalized_name = urllib.parse.quote(product.name)
        product.image_url = f"/static/img/cereal/{normalized_name}.jpg"

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
