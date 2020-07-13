import Layout from "components/layout";
import { GetServerSideProps } from "next";
import getPortfolioPosition, {
  PortfolioPosition,
} from "api/portfolio-position";

interface PositionProps {
  portfolioPortfolio: PortfolioPosition | null;
}

const Position = ({ portfolioPortfolio }: PositionProps) => {
  return (
    <Layout title="Posição">
      <div className="uk-flex uk-flex-right">
        <div className="uk-flex-1">
          <h1 className="uk-heading-line">
            <span>Posição</span>
          </h1>

          {portfolioPortfolio && (
            <>
              <div className="uk-card">
                <div className="uk-card-title">Total</div>
                <div className="uk-card-body">{portfolioPortfolio.amount}</div>
              </div>

              <div className="uk-card">
                <div className="uk-card-title">Por Ativo</div>
                <table className="uk-card-body uk-table">
                  <tbody>
                    {portfolioPortfolio.assets.map((asset, index) => (
                      <tr key={index}>
                        <td>
                          {asset.assetable[0] === "treasury"
                            ? "Tesouro SELIC"
                            : `ETF ${asset.assetable[1]}`}
                        </td>
                        <td>{asset.amount}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </>
          )}
        </div>
        <div className="uk-flex-none">
          <h1 className="uk-margin-left">
            <span>
              Porti<span className="uk-text-primary">folio</span>
            </span>
          </h1>
        </div>
      </div>
    </Layout>
  );
};

export const getServerSideProps: GetServerSideProps<PositionProps> = async ({
  req: {
    headers: { cookie },
  },
}) => {
  if (!cookie) {
    throw "MissingCookie";
  }

  return {
    props: {
      portfolioPortfolio: await getPortfolioPosition(cookie),
    },
  };
};

export default Position;
