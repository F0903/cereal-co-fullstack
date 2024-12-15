export function assertNotNull<T>(value: T | null | undefined): T {
  if (!value) {
    throw "value was null.";
  }
  return value;
}
