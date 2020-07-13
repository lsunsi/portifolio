import fetch from "isomorphic-unfetch";
import * as dec from "decoders";

const importTrades = (file: Blob): Promise<void> =>
  fetch("http://localhost:8000/import-trades", {
    credentials: "include",
    method: "POST",
    body: file,
  }).then((resp) => {
    if (!resp.ok) {
      throw "Bad status";
    }
  });

export default importTrades;
