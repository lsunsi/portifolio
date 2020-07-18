import Layout from "components/layout";
import getTransactions, { Assetable, Transaction } from "api/transactions";
import { GetServerSideProps } from "next";
import { useReducer } from "react";

type Props = { transactions: Transaction[] };

type State =
  | ["unfiltered"]
  | ["filtered-year", number]
  | ["filtered-type", string];

type Action = ["filter-year", number] | ["filter-type", string];

const initialState: State = ["unfiltered"];

const reducer = (_: State, action: Action): State => {
  switch (action[0]) {
    case "filter-year":
      return ["filtered-year", action[1]];
    case "filter-type":
      return ["filtered-type", action[1]];
  }
};

const formateDate = (millis: number): string =>
  new Date(millis).toLocaleDateString();

const formatAssetable = (assetable: Assetable): string => {
  switch (assetable.type) {
    case "Etf":
      return `ETF ${assetable.data}`;
    case "Treasury":
      return `LFT ${assetable.data}`;
  }
};

const yearFilters = (transactions: Transaction[]): number[] => [
  ...new Set(transactions.map((t) => new Date(t.date).getFullYear())),
];

const assetTypeFilters = (transactions: Transaction[]): string[] => [
  ...new Set(transactions.map((t) => t.assetable.type)),
];

const filteredTransactions = (
  transactions: Transaction[],
  state: State
): Transaction[] => {
  switch (state[0]) {
    case "unfiltered":
      return transactions;
    case "filtered-type":
      return transactions.filter((t) => t.assetable.type == state[1]);
    case "filtered-year":
      return transactions.filter(
        (t) => new Date(t.date).getFullYear() == state[1]
      );
  }
};

const Transactions = ({ transactions }: Props) => {
  const [state, dispatch] = useReducer(reducer, initialState);

  return (
    <Layout title="Transações">
      <div className="uk-flex">
        <ul className="uk-nav uk-nav-default uk-width-1-6">
          <li className="uk-nav-header">Filtros</li>
          {yearFilters(transactions).map((year) => (
            <li
              className={`${
                state[0] == "filtered-year" && state[1] == year
                  ? "uk-active"
                  : ""
              }`}
            >
              <a onClick={() => dispatch(["filter-year", year])}>{year}</a>
            </li>
          ))}
          <li className="uk-nav-divider uk-width-1-2"></li>
          {assetTypeFilters(transactions).map((type) => (
            <li
              className={`${
                state[0] == "filtered-type" && state[1] == type
                  ? "uk-active"
                  : ""
              }`}
            >
              <a onClick={() => dispatch(["filter-type", type])}>{type}</a>
            </li>
          ))}
        </ul>
        <div className="uk-width-full">
          <table className="uk-table uk-table-small">
            <thead>
              <tr>
                <th className="uk-text-right">Ativo</th>
                <th className="uk-text-right">Data</th>
                <th className="uk-text-right">Quantidade</th>
                <th className="uk-text-right">Preço</th>
                <th className="uk-text-right">Valor</th>
              </tr>
            </thead>
            <tbody>
              {filteredTransactions(transactions, state).map(
                ({ price, quantity, amount, date, assetable }) => (
                  <tr>
                    <td className="uk-text-right">
                      {formatAssetable(assetable)}
                    </td>
                    <td className="uk-text-right">{formateDate(date)}</td>
                    <td className="uk-text-right">{quantity.toFixed(8)}</td>
                    <td className="uk-text-right">{price.toFixed(8)}</td>
                    <td className="uk-text-right">{amount.toFixed(2)}</td>
                  </tr>
                )
              )}
            </tbody>
          </table>
        </div>
      </div>
    </Layout>
  );
};

export const getServerSideProps: GetServerSideProps<Props> = async (ctx) => {
  if (!ctx.req.headers.cookie) {
    throw "MissingCookie";
  }

  const transactions = await getTransactions(ctx.req.headers.cookie);
  return { props: { transactions } };
};

export default Transactions;
