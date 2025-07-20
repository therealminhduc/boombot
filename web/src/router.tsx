import { createRouter, createRoute, createRootRoute } from "@tanstack/react-router";
import Home from "./pages/Home";
import Contribution from "./pages/Contribution";
import Admin from "./pages/Admin";
import ProtectedRoute from "./components/ProtectedRoute";
import AdminConnection from "./pages/AdminConnection";

const rootRoute = createRootRoute();

const indexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/",
    component: Home,
});

const contributionRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/contribution",
    component: Contribution,
});

const adminRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/admin",
    component: () => (  // Wrap Admin with ProtectedRoute
        <ProtectedRoute>
            <Admin />
        </ProtectedRoute>
    ),
});

const adminConnectionRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/admin/connection",
    component: AdminConnection,
});

const routeTree = rootRoute.addChildren([indexRoute, contributionRoute, adminRoute, adminConnectionRoute]);

export const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
    interface Register {
        router: typeof router;
    }
}