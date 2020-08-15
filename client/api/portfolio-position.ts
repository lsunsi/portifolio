import fetch from "isomorphic-unfetch";
import * as dec from "decoders";

type Assetable =
  | { type: "Etf"; data: string }
  | {
      type: "TreasuryBond";
      data: string;
    };

interface AssetPosition {
  assetable: Assetable;
  quantity: number;
  amount: number;
  price: number;
}

export interface PortfolioPosition {
  assets: AssetPosition[];
  amount: number;
}

const assetableDecoder: dec.Decoder<Assetable> = dec.object({
  type: dec.oneOf<"TreasuryBond" | "Etf">(["TreasuryBond", "Etf"]),
  data: dec.string,
});

const assetPositionDecoder: dec.Decoder<AssetPosition> = dec.object({
  assetable: assetableDecoder,
  quantity: dec.number,
  amount: dec.number,
  price: dec.number,
});

const portfolioPositionDecoder: dec.Decoder<PortfolioPosition> = dec.object({
  assets: dec.array(assetPositionDecoder),
  amount: dec.number,
});

const decode = dec.guard(portfolioPositionDecoder);

const getPortfolioPosition = (cookie: string): Promise<PortfolioPosition> =>
  fetch(`${process.env.NEXT_PUBLIC_SERVER_URL}/portfolio-position`, {
    headers: { cookie },
  })
    .then((resp) => {
      if (resp.status == 200) {
        return resp.json();
      } else {
        throw "BadStatus";
      }
    })
    .then(decode);

export default getPortfolioPosition;
