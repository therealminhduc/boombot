import { createRouter, createRoute, createRootRoute } from "@tanstack/react-router";
import Home from "./pages/Home";
import Contribution from "./pages/Contribution";

const rootRoute = createRootRoute();

const indexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/",
    component: Home,
})

const contributionRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/contribution",
    component: Contribution,
})

const routeTree = rootRoute.addChildren([indexRoute, contributionRoute]);

export const router = createRouter({ routeTree });

declare module "@tanstack/react-router" {
    interface Register {
        router: typeof router;
    }
}