import fetch from "isomorphic-unfetch";

const importTrades = (file: Blob): Promise<void> =>
  fetch(`${process.env.NEXT_PUBLIC_SERVER_URL}/import-trades`, {
    credentials: "include",
    method: "POST",
    body: file,
  }).then((resp) => {
    if (!resp.ok) {
      throw "Bad status";
    }
  });

export default importTrades;
