let idCounter = 0;

export function getData(count: number): { id: string; text: string; }[] {
  const data = [];
  for (let index = 0; index < count; index++) {
    data.push({
      id: String(idCounter++),
      text: "Connecting...",
    });
  }
  return data;
}