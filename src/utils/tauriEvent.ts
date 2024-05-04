import { Event, UnlistenFn, listen } from "@tauri-apps/api/event";
import { onMounted, onUnmounted } from "vue";

export function useTauriEvent(
  event: string,
  cb: (event: Event<unknown>) => void
) {
  let unlisten: UnlistenFn | null = null;
  onMounted(async () => {
    unlisten = await listen(event, (event) => {
      cb(event);
    });
  });
  
  onUnmounted(() => {
    if (unlisten) {
      unlisten();
    }
  });
}
