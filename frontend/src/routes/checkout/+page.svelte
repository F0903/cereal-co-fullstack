<script lang="ts">
  import { getFullImageUrl } from "$lib/api/utils";
  import Button from "$lib/Button.svelte";
  import Image from "$lib/Image.svelte";
  import Spacer from "$lib/Spacer.svelte";
  import { faCheck } from "@fortawesome/free-solid-svg-icons";
  import type { PageData } from "./$types";
  import InputField from "$lib/InputField.svelte";
  import { assertNotNull } from "$lib/utils/typeUtils";
  import { addOrder, Order, OrderItem } from "$lib/api/orders";
  import { goto } from "$app/navigation";
  import { clearCart } from "$lib/cart/localCartApi";
  import Table from "$lib/Table.svelte";

  let { data }: { data: PageData } = $props();

  let form: HTMLFormElement;

  let totalPrice = $derived.by(() => {
    const sum = data.cart.calcSum().toFixed(2);
    return sum;
  });

  async function onCheckoutClick() {
    const formData = new FormData(form);

    const cartItems = data.cart.getItems();
    const orderItems: OrderItem[] = [];
    for (const item of cartItems) {
      orderItems.push({
        product_id: item.product.id,
        quantity: item.quantity,
      });
    }

    const order: Order = {
      shipping_name: assertNotNull(formData.get("Name") as string),
      shipping_phone: assertNotNull(formData.get("Phone") as string),
      shipping_mail: assertNotNull(formData.get("Mail") as string),
      shipping_address: assertNotNull(formData.get("Address") as string),
      total: parseFloat(totalPrice),
      order_items: orderItems,
    };

    const orderId = await addOrder(order);
    clearCart();
    await goto(`/checkout/success/${orderId}`);
  }
</script>

<div class="checkout">
  <h1>Checkout</h1>
  <Spacer --margin-bottom="50px" --width="10%" />

  <Table>
    <thead>
      <tr>
        <th>Image</th>
        <th>Name</th>
        <th>Quantity</th>
        <th>Price</th>
        <th>Total</th>
      </tr>
    </thead>
    <tbody>
      {#each data.cart.getItems() as item}
        <tr>
          <td>
            <Image
              src={getFullImageUrl(item.product.image_url)}
              alt="Image of {item.product.name}"
              --width="50px"
              --height="50px"
              --border-radius="7px"
            />
          </td>
          <td>{item.product.name}</td>
          <td>{item.quantity}</td>
          <td>${item.product.price}</td>
          <td>{item.quantity * item.product.price}</td>
        </tr>
      {/each}
    </tbody>
  </Table>

  <Spacer --margin-bottom="50px" --width="10%" />

  <form class="shipping-form" bind:this={form}>
    <InputField name="Name" />
    <InputField name="Phone" />
    <InputField name="Mail" />
    <InputField name="Address" />
  </form>

  <div class="checkout-price-container">
    <span class="total-price">${totalPrice}</span>
    <Spacer --margin-bottom="5px" />
    <Button text="Purchase" prefixIcon={faCheck} onclick={onCheckoutClick} />
  </div>
</div>

<style>
  .shipping-form {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 10px;
    margin-bottom: 15px;
  }

  .total-price {
    font-size: 1.5em;
    font-weight: 700;
  }

  .checkout-price-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;

    background-color: var(--primary-color);

    padding: 15px;
    border-radius: 10px;
  }

  .checkout {
    padding: 50px;
    background-color: var(--secondary-color);
    margin-top: 100px;
    margin-left: auto;
    margin-right: auto;
    border-radius: 25px;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    max-width: 1000px;
  }
</style>
