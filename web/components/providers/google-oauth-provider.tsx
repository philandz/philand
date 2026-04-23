"use client";

import { GoogleOAuthProvider } from "@react-oauth/google";

export function GoogleAuthProvider({ children }: { children: React.ReactNode }) {
  const envClientId = process.env.NEXT_PUBLIC_GOOGLE_CLIENT_ID;
  const clientId = envClientId || "NOT_CONFIGURED";

  if (!envClientId) {
    console.warn("NEXT_PUBLIC_GOOGLE_CLIENT_ID is not set - Using dummy ID. Google Sign-In will fail if used.");
  }

  return (
    <GoogleOAuthProvider clientId={clientId}>
      {children}
    </GoogleOAuthProvider>
  );
}
