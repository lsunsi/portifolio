import fetch from "isomorphic-unfetch";
import * as dec from "decoders";

export type PortfolioAmount = {
  grossTotal: number;
  invested: number;
  date: number;
};

const portfolioAmountDecoder = dec.map(
  dec.tuple3(dec.string, dec.number, dec.number),
  ([dateString, invested, grossTotal]): PortfolioAmount => ({
    date: Date.parse(dateString),
    grossTotal,
    invested,
  })
);

const decode = dec.guard(dec.array(portfolioAmountDecoder));

const portfolioAmounts = (cookie: string): Promise<PortfolioAmount[]> =>
  fetch("http://localhost:8000/portfolio-amounts", {
    headers: { cookie },
  })
    .then((resp) => resp.json())
    .then(decode);

export default portfolioAmounts;
