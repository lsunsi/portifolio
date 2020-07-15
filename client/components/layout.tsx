import Base from "./base";

type LayoutProps = {
  title: string;
  children: React.ReactNode;
};

const Layout = ({ title, children }: LayoutProps) => (
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
          <span>
            Porti<span className="uk-text-primary">folio</span>
          </span>
        </h1>
      </div>
    </div>
  </Base>
);

export default Layout;
