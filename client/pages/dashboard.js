
import React, { useState, useEffect } from 'react';
import useWallet from '../hooks/useWallet';

export default function Dashboard() {
  const { wallet, connect, error } = useWallet();
  const [status, setStatus] = useState('');
  const [claimed, setClaimed] = useState(0);
  const [unclaimed, setUnclaimed] = useState(0);

  // Get session ID (same as mining)
  function getSessionId() {
    if (typeof window === 'undefined') return '';
    let sessionId = localStorage.getItem('nun_session_id');
    if (!sessionId) {
      sessionId = Math.random().toString(36).substring(2) + Date.now();
      localStorage.setItem('nun_session_id', sessionId);
    }
    return sessionId;
  }

  // Fetch unclaimed NUNs
  const fetchUnclaimed = async () => {
    try {
      const res = await fetch(`http://localhost:3030/unclaimed?session_id=${getSessionId()}`);
      if (res.ok) {
        const data = await res.json();
        setUnclaimed(data.unclaimed || 0);
      }
    } catch {}
  };

  useEffect(() => {
    fetchUnclaimed();
  }, []);

  const handleClaim = async () => {
    if (!wallet) {
      setStatus('Please connect your wallet first.');
      return;
    }
    setStatus('Claiming...');
    try {
      const res = await fetch('http://localhost:3030/claim', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ session_id: getSessionId(), wallet_address: wallet }),
      });
      if (res.ok) {
        const data = await res.json();
        setClaimed(data.claimed_amount);
        setStatus(data.status);
        fetchUnclaimed();
      } else {
        setStatus('Claim failed.');
      }
    } catch (e) {
      setStatus('Error connecting to server.');
    }
  };

  return (
    <div style={{marginTop: 80, padding: 32}}>
      <h2>UnifiedNUN Dashboard</h2>
      <p>Connect your wallet to claim your mined NUN.</p>
      <div style={{marginBottom: 16}}>
        {wallet ? (
          <span style={{color: 'green'}}>Wallet connected: {wallet}</span>
        ) : (
          <button onClick={connect} style={{padding: '8px 16px', background: '#eee', border: '1px solid #ccc', borderRadius: 4, fontWeight: 'bold', cursor: 'pointer'}}>Connect Wallet</button>
        )}
        {error && <div style={{color: 'red', marginTop: 8}}>{error}</div>}
      </div>
      <div style={{marginBottom: 16}}>
        <b>Unclaimed NUN:</b> {unclaimed}
      </div>
      <button onClick={handleClaim} style={{padding: '8px 16px', background: '#ffd700', border: 'none', borderRadius: 4, fontWeight: 'bold', cursor: 'pointer'}}>
        Claim NUN
      </button>
      {status && <div style={{marginTop: 16}}>{status}</div>}
      {claimed > 0 && <div style={{marginTop: 8}}>Claimed: {claimed} NUN</div>}
    </div>
  );
}
