
import '../styles/global.css';
import useActionMining from '../hooks/useActionMining';

export default function App({ Component, pageProps }) {
  useActionMining();
  return <Component {...pageProps} />;
}
