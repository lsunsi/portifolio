import Head from "next/head";

interface LayoutProps {
  children: React.ReactNode;
  title: string;
}

const Base = ({ title, children }: LayoutProps) => (
  <>
    <Head>
      <meta
        name="viewport"
        content="width=device-width, initial-scale=1, shrink-to-fit=no"
      />
      <link
        rel="stylesheet"
        href="https://cdn.jsdelivr.net/npm/uikit@3.5.4/dist/css/uikit.min.css"
      />
      <script src="https://cdn.jsdelivr.net/npm/uikit@3.5.4/dist/js/uikit-icons.min.js" />
      <script src="https://cdn.jsdelivr.net/npm/uikit@3.5.4/dist/js/uikit.min.js" />
      <title>Portifolio | {title}</title>
    </Head>

    <div className="uk-height-1-1 uk-padding">{children}</div>
  </>
);

export default Base;
