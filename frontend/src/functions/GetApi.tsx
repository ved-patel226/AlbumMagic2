export default async function fetchAPI(url: string) {
  try {
    return await fetch(`http://127.0.0.1:3000/${url}`)
      .then((res) => res.json())
      .then((data) => data);
  } catch (error) {
    console.error("Error fetching API:", error);
    throw error;
  }
}
