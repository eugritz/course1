import { onMounted, onUnmounted, ref } from "vue";
import { debounce as debounceFunc } from "./debounce";

interface WindowSizeOptions {
  debounce?: number,
}

export function useWindowSize({
  debounce = 0,
}: WindowSizeOptions = {}) {
  const width = ref(window.innerWidth);
  const height = ref(window.innerHeight);

  function updateValues(innerWidth: number, innerHeight: number) {
    width.value = innerWidth;
    height.value = innerHeight;
  }

  const update = debounce > 0
    ? debounceFunc(updateValues, debounce)
    : updateValues;

  function handleResize(this: Window, _event: UIEvent) {
    update(this.innerWidth, this.innerHeight);
  }

  onMounted(() => {
    window.addEventListener("resize", handleResize);
  });

  onUnmounted(() => {
    window.removeEventListener("resize", handleResize);
  });

  return { width, height };
}
