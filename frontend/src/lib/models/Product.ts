export const product_types = ["Product", "Clothing", "Book"];

// Try to find a more generic solution for this instead of hardcoding it.
export function getAttributesOfProductType(typ: string) {
    switch (typ) {
        case "Product":
            return Product.getAttributeNames();
        case "Clothing":
            return Clothing.getAttributeNames();
        case "Book":
            return Book.getAttributeNames();
        default:
            // Don't throw here, instead return empty so the placeholder works.
            return [];
    }
}

export function constructProductFromAttributeList(
    product_type: string,
    attributes: Attribute[]
): Product {
    let product;
    switch (product_type) {
        case "Product":
            product = new Product();
            break;
        case "Clothing":
            product = new Clothing();
            break;
        case "Book":
            product = new Book();
        default:
            throw new Error(`Unknown product type: ${product_type}`);
    }

    attributes.forEach((attr) => {
        product[attr.Name] = attr.Value;
    });

    return product;
}

export class Product {
    [k: string]: string | number;
    Name!: string;
    Description!: string;
    Price!: string;
    Quantity!: number;

    public static getAttributeNames() {
        return ["Name", "Description", "Price", "Quantity"];
    }
}

export class Clothing extends Product {
    Color!: string;
    Material!: string;
    Size!: string;

    public static getAttributeNames() {
        return [...super.getAttributeNames(), "Color", "Material", "Size"];
    }
}

export class Book extends Product {
    Genre!: string;
    PageCount!: number;

    public static getAttributeNames() {
        return [...super.getAttributeNames(), "Genre", "PageCount"];
    }
}
