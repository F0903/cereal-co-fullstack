import type { Action } from "svelte/action";

export const clickOutside: Action<
  HTMLElement,
  undefined,
  { onclickoutside: (event: MouseEvent) => void }
> = (node: Node) => {
  const handleClick = (event: MouseEvent) => {
    if (
      node &&
      !node.contains(event.target as Node) &&
      !event.defaultPrevented
    ) {
      node.dispatchEvent(new CustomEvent("clickoutside", node as any));
    }
  };

  $effect(() => {
    document.addEventListener("click", handleClick, true);

    return () => {
      document.removeEventListener("click", handleClick, true);
    };
  });
};
