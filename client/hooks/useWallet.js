import { useState } from 'react';

export default function useWallet() {
  const [wallet, setWallet] = useState(null);
  const [error, setError] = useState(null);

  const connect = async () => {
    setError(null);
    if (window.ethereum) {
      try {
        const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
        setWallet(accounts[0]);
      } catch (err) {
        setError('Wallet connection rejected.');
      }
    } else {
      setError('MetaMask not found. Please install MetaMask.');
    }
  };

  return { wallet, connect, error };
}
