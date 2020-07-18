import Base from "components/base";
import Link from "next/link";
import { GetServerSideProps } from "next";

type Props = {
  hasCookie: boolean;
};

const Index = ({ hasCookie }: Props) => (
  <Base title="Olá!">
    <h1 className="uk-heading-medium uk-heading-line uk-text-center">
      <span>
        Porti<span className="uk-text-primary">folio</span>
      </span>
    </h1>

    <ul className="uk-nav uk-nav-primary uk-text-center">
      {hasCookie ? (
        <>
          <li>
            <Link href="/position">
              <a>Posição</a>
            </Link>
          </li>
          <li>
            <Link href="/history">
              <a>Histórico</a>
            </Link>
          </li>
          <li>
            <Link href="/transactions">
              <a>Transações</a>
            </Link>
          </li>
        </>
      ) : (
        <li>
          <Link href="/import">
            <a>Importar</a>
          </Link>
        </li>
      )}
    </ul>
  </Base>
);

export const getServerSideProps: GetServerSideProps<Props> = async (ctx) => ({
  props: { hasCookie: !!ctx.req.headers.cookie },
});

export default Index;
