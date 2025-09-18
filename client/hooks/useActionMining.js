import { useEffect } from 'react';

// Utility to generate or retrieve a session ID (anonymous user tracking)
function getSessionId() {
  let sessionId = localStorage.getItem('nun_session_id');
  if (!sessionId) {
    sessionId = Math.random().toString(36).substring(2) + Date.now();
    localStorage.setItem('nun_session_id', sessionId);
  }
  return sessionId;
}

export default function useActionMining(onMined) {
  useEffect(() => {
    const sessionId = getSessionId();
    const mine = async (action) => {
      try {
        const res = await fetch('http://localhost:3030/mine', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ session_id: sessionId, action }),
        });
        if (res.ok) {
          const data = await res.json();
          if (onMined) onMined(data);
        }
      } catch (e) {
        // Handle error (optional)
      }
    };

    // List of user actions to listen for
    const actions = [
      { event: 'click', name: 'click' },
      { event: 'scroll', name: 'scroll' },
      { event: 'keydown', name: 'keydown' },
      { event: 'mousemove', name: 'mousemove' },
      { event: 'visibilitychange', name: 'visibilitychange' },
    ];

    // Handler to mine on each action
    const handlers = actions.map(({ event, name }) => {
      const handler = () => mine(name);
      window.addEventListener(event, handler);
      return { event, handler };
    });

    // Initial page load mines 'visit'
    mine('visit');

    return () => {
      handlers.forEach(({ event, handler }) => {
        window.removeEventListener(event, handler);
      });
    };
  }, [onMined]);
}
