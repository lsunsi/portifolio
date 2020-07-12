import './main.css';
import { Elm } from './Main.elm';

const {localStorage} = window;

const localStoragePortfolioIdKey = 'portfolio-id';
const portfolioId = localStorage.getItem(localStoragePortfolioIdKey);

const app = Elm.Main.init({
  node: document.getElementById('root'),
  flags: portfolioId && parseInt(portfolioId)
});

app.ports.storePortfolioId.subscribe(id => localStorage.setItem(localStoragePortfolioIdKey, id));
