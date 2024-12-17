export const minuteToMillis = (minutes: number) => minutes * 60 * 1000;
export const dateOffset = (minutes: number) =>
    Date.now() + minuteToMillis(minutes);
