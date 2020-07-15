import Layout from "components/layout";
import { useReducer, useEffect } from "react";
import importTrades from "api/import-trades";
import { useRouter, NextRouter } from "next/router";

type ImportAction = ["select", Blob] | ["send"];

type ImportState = [
  ["waiting"] | ["selected", Blob] | ["sent"],
  null | ((_: NextRouter) => void)
];

const initialState: ImportState = [["waiting"], null];

const reducer = ([state]: ImportState, action: ImportAction): ImportState => {
  switch (action[0]) {
    case "select":
      return [["selected", action[1]], null];
    case "send":
      if (state[0] == "selected") {
        return [
          ["sent"],
          async (router) => {
            await importTrades(state[1]);
            router.push("/position");
          },
        ];
      } else {
        return [state, null];
      }
  }
};

const Import = () => {
  const [[state, cmd], dispatch] = useReducer(reducer, initialState);
  const router = useRouter();

  useEffect(() => {
    if (cmd) {
      cmd(router);
    }
  }, [cmd]);

  return (
    <Layout title="Importar">
      <div className="js-upload" data-uk-form-custom>
        <input
          onChange={(e) =>
            e.target.files && dispatch(["select", e.target.files[0]])
          }
          accept="text/csv"
          type="file"
        />
        <button
          className="uk-button uk-button-default"
          disabled={state[0] == "sent"}
          type="button"
        >
          {state[0] == "waiting" ? "Selecionar" : "Reselecionar"}
        </button>
      </div>
      <button
        className="uk-button uk-button-primary"
        onClick={() => dispatch(["send"])}
        disabled={state[0] != "selected"}
        type="button"
      >
        Enviar
      </button>
    </Layout>
  );
};

export default Import;
