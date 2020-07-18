import fetch from "isomorphic-unfetch";
import * as dec from "decoders";

export type Assetable =
  | { type: "Treasury"; data: string }
  | { type: "Etf"; data: string };

export interface Transaction {
  assetable: Assetable;
  date: number;
  price: number;
  quantity: number;
  amount: number;
}

const dateDecoder: dec.Decoder<number> = dec.compose(
  dec.map(dec.string, Date.parse),
  dec.predicate(isFinite, "Invalid Date")
);

const assetableDecoder: dec.Decoder<Assetable> = dec.object({
  type: dec.oneOf<"Treasury" | "Etf">(["Treasury", "Etf"]),
  data: dec.string,
});

const transactionDecoder: dec.Decoder<Transaction> = dec.object({
  assetable: assetableDecoder,
  date: dateDecoder,
  price: dec.number,
  quantity: dec.number,
  amount: dec.number,
});

const decode = dec.guard(dec.array(transactionDecoder));

const getTransactions = (cookie: string): Promise<Transaction[]> =>
  fetch("http://localhost:8000/transactions", { headers: { cookie } })
    .then((resp) => {
      if (resp.status == 200) {
        return resp.json();
      } else {
        throw "BadStatus";
      }
    })
    .then(decode);

export default getTransactions;
