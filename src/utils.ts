export function splitText(text: string, limit: number = 4096): string[] {
    const parts: string[] = [];
    let currentPart = "";
    const lines = text.split('\n');

    for (const line of lines) {
        if (currentPart.length + line.length + 1 > limit) {
            parts.push(currentPart);
            currentPart = line;
        } else {
            if (currentPart) {
                currentPart += "\n";
            }
            currentPart += line;
        }
    }
    if (currentPart) {
        parts.push(currentPart);
    }

    return parts;
}