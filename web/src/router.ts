import { createRouter, createRoute, createRootRoute } from "@tanstack/react-router";
import Home from "./pages/Home";
import Contribution from "./pages/Contribution";
import Admin from "./pages/Admin";

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
    component: Admin,
})

const routeTree = rootRoute.addChildren([indexRoute, contributionRoute, adminRoute]);

export const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
    interface Register {
        router: typeof router;
    }
}