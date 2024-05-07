export function debounce(fn: (...args: any[]) => any, timeout: number) {
  let timer: Timer;
  return function(...args: any[]) {
    if (timer) {
       clearTimeout(timer);
    }

    // @ts-ignore 2683
    const context = this;
    timer = setTimeout(() => {
      fn.apply(context, args);
    }, timeout);
  }
}
