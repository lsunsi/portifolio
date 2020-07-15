import Layout from "components/layout";
import { LineChart, Line, YAxis, XAxis, Tooltip, Legend } from "recharts";
import { GetServerSideProps } from "next";
import portfolioAmounts, { PortfolioAmount } from "api/portfolio-amounts";

type HistoryProps = {
  amounts: PortfolioAmount[];
};

const History = ({ amounts }: HistoryProps) => (
  <Layout title="Histórico">
    <LineChart width={800} height={400} data={amounts}>
      <Legend />
      <Tooltip />
      <YAxis />
      <XAxis dataKey="date" />
      <Line dataKey="grossTotal" stroke="blue" dot={false} />
      <Line dataKey="invested" stroke="red" dot={false} />
    </LineChart>
  </Layout>
);

export const getServerSideProps: GetServerSideProps<HistoryProps> = async (
  ctx
) => {
  if (!ctx.req.headers.cookie) {
    throw "MissingCookie";
  }

  return {
    props: { amounts: await portfolioAmounts(ctx.req.headers.cookie) },
  };
};

export default History;
