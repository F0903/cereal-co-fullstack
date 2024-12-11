export function getCookies(): { [key: string]: string } {
    // Ask ChatGPT
    const cookies = document.cookie
        .split("; ")
        .reduce<{ [key: string]: string }>((acc, cookie) => {
            const [key, value] = cookie.split("=");
            acc[key] = decodeURIComponent(value);
            return acc;
        }, {});

    return cookies;
}

export function getCookie(name: string): string {
    return getCookies()[name];
}
