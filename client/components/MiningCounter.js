
import React, { useState, useEffect } from 'react';
import useActionMining from '../hooks/useActionMining';

function getSessionId() {
  if (typeof window === 'undefined') return '';
  let sessionId = localStorage.getItem('nun_session_id');
  if (!sessionId) {
    sessionId = Math.random().toString(36).substring(2) + Date.now();
    localStorage.setItem('nun_session_id', sessionId);
  }
  return sessionId;
}

export default function MiningCounter() {
  const [unclaimed, setUnclaimed] = useState(0);
  const sessionId = getSessionId();

  // Short explanation as notice
  const notice = `Every action you take (visit, scroll, click, open, close, create, etc.) automatically mines NUN for you. 50% goes to you, 50% to the platform. Claim your mined NUNs anytime!`;

  // Fetch unclaimed NUNs from backend
  const fetchUnclaimed = async () => {
    try {
      const res = await fetch(`http://localhost:3030/unclaimed?session_id=${sessionId}`);
      if (res.ok) {
        const data = await res.json();
        setUnclaimed(data.unclaimed || 0);
      }
    } catch {}
  };

  useEffect(() => {
    fetchUnclaimed();
    // Optionally, poll every 30s: const interval = setInterval(fetchUnclaimed, 30000);
    // return () => clearInterval(interval);
  }, [sessionId]);

  // Update unclaimed count after mining event
  useActionMining(() => {
    fetchUnclaimed();
  });

  // Claim button handler
  const handleClaim = () => {
    window.location.href = '/dashboard';
  };

  return (
    <div style={{position: 'fixed', top: 0, left: 0, width: '100%', background: '#222', color: '#fff', padding: '10px', zIndex: 1000, display: 'flex', alignItems: 'center', justifyContent: 'space-between'}}>
      <span>� Unclaimed NUN: {unclaimed}</span>
      <span style={{marginLeft: 20, fontSize: '0.95em'}}>{notice}</span>
      <button onClick={handleClaim} style={{marginLeft: 20, padding: '8px 16px', background: '#ffd700', border: 'none', borderRadius: 4, fontWeight: 'bold', cursor: 'pointer'}}>Claim Your Mined NUN</button>
    </div>
  );
}
