"use client";

import { QueryClientProvider } from "@tanstack/react-query";
import { ReactNode, useState } from "react";
import { ReactQueryDevtools } from "@tanstack/react-query-devtools";
import { GoogleAuthProvider } from "@/components/providers/google-oauth-provider";

import { getQueryClient } from "@/lib/queryClient";
import { ThemeProvider } from "@/lib/theme-provider";
import { Toaster } from "@/components/ui/sonner";

interface ProvidersProps {
  readonly children: ReactNode;
}

export function AppProviders({ children }: Readonly<ProvidersProps>) {
  const [queryClient] = useState(() => getQueryClient());


  return (
    <ThemeProvider>
      <GoogleAuthProvider>
        <QueryClientProvider client={queryClient}>
          {children}
          <ReactQueryDevtools initialIsOpen={false} />
          <Toaster />
        </QueryClientProvider>
      </GoogleAuthProvider>
    </ThemeProvider>
  );
}
