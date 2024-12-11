export function findParentWithTag(
    tag: string,
    elem: HTMLElement
): HTMLElement | null {
    if (elem.tagName.toLowerCase() === tag) {
        return elem;
    }

    const parent = elem.parentElement;
    if (!parent) {
        return null;
    }
    return findParentWithTag(tag, parent);
}

export function getObjectFromFormData<T>(form_element: HTMLFormElement) {
    let form = findParentWithTag("form", form_element);

    // Construct form data from the form html element.
    const formData = new FormData(form as HTMLFormElement);

    // Get the entries in the form data wrapped in an object, so we can serialize it to JSON correctly
    const entriesAsObject = Object.fromEntries(formData) as unknown as T;

    return entriesAsObject;
}
