import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import { RouterProvider } from '@tanstack/react-router'
import { router } from './router'
import { Toaster } from "@/components/ui/sonner";
import { AuthProvider } from './contexts/AuthContext'
import { ThemeProvider } from '@/components/theme-provider'

createRoot(document.getElementById('root')!).render(
  <ThemeProvider defaultTheme="system" storageKey="vite-ui-theme">
    <StrictMode>
      <AuthProvider>
        <RouterProvider router={router} />
        <Toaster />
      </AuthProvider>
    </StrictMode>
  </ThemeProvider>,
)
