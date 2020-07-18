import Base from "./base";
import Link from "next/link";
import { useRouter } from "next/router";

type LayoutProps = {
  title: string;
  children: React.ReactNode;
};

const Layout = ({ title, children }: LayoutProps) => {
  const { pathname } = useRouter();

  return (
    <Base title={title}>
      <div className="uk-flex uk-flex-right">
        <div className="uk-flex-1">
          <h1 className="uk-heading-line">
            <span>{title}</span>
          </h1>
          {children}
        </div>

        <div className="uk-flex-none">
          <h1 className="uk-margin-left">
            <Link href="/">
              <span>
                Porti<span className="uk-text-primary">folio</span>
              </span>
            </Link>
          </h1>

          <ul className="uk-nav uk-nav-primary uk-text-right">
            <li className={`${pathname == "/position" ? "uk-active" : ""}`}>
              <Link href="/position">
                <a>Posição</a>
              </Link>
            </li>
            <li className={`${pathname == "/history" ? "uk-active" : ""}`}>
              <Link href="/history">
                <a>Histórico</a>
              </Link>
            </li>
            <li className={`${pathname == "/transactions" ? "uk-active" : ""}`}>
              <Link href="/transactions">
                <a>Transações</a>
              </Link>
            </li>
          </ul>
        </div>
      </div>
    </Base>
  );
};

export default Layout;
