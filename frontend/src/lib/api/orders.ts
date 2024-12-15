import { PUBLIC_BACKEND_URL } from "$env/static/public";

// Returns the created orders id if successful
export async function addOrder(order: Order): Promise<number> {
  const resp = await fetch(`${PUBLIC_BACKEND_URL}/api/v1/orders`, {
    method: "POST",
    body: JSON.stringify(order),
    headers: {
      "Content-Type": "application/json",
      // We need a CSRF token for state changing methods.
      //"X-CSRF-TOKEN": getCookie("csrf_access_token"),
    },
    //credentials: "include",
  });
  if (!resp.ok) {
    throw new Error(`Response was not OK. Response was:\n${resp.statusText}`);
  }

  const json: { id: number } = await resp.json();
  return json.id;
}

export async function getOrder(id: number): Promise<Order> {
  const resp = await fetch(`${PUBLIC_BACKEND_URL}/api/v1/orders/${id}`, {
    method: "GET",
    //credentials: "include",
  });
  if (!resp.ok) {
    throw new Error(`Response was not OK. Response was:\n${resp.statusText}`);
  }

  const order = resp.json();
  return order;
}
