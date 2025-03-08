import api from "@/services/api";
import { useUserStore } from "@/stores/user";
import { CacheKeys, cache } from "@/utils/cache";
import { createPinia, setActivePinia } from "pinia";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

// Mock dependencies
vi.mock("@/services/api", () => ({
  default: {
    get: vi.fn(),
  },
}));

vi.mock("@/utils/cache", () => ({
  CacheKeys: {
    USER: "user",
    LINKS: "links",
    SETTINGS: "settings",
  },
  cache: {
    get: vi.fn(),
    set: vi.fn(),
    clear: vi.fn(),
  },
}));

// Mock the other stores that are used in userStore
vi.mock("@/stores/links", () => ({
  useLinksStore: vi.fn().mockReturnValue({
    $reset: vi.fn(),
    $patch: vi.fn(),
  }),
}));

vi.mock("@/stores/settings", () => ({
  useUserSettingsStore: vi.fn().mockReturnValue({
    $reset: vi.fn(),
    $patch: vi.fn(),
  }),
}));

describe("User Store", () => {
  let store: ReturnType<typeof useUserStore>;

  beforeEach(() => {
    // Create a fresh pinia instance and make it active for this test
    setActivePinia(createPinia());
    store = useUserStore();

    // Mock store methods that are called internally
    store.$reset = vi.fn();
    store.$patch = vi.fn();

    // Reset all mocks
    vi.clearAllMocks();
  });

  afterEach(() => {
    vi.resetAllMocks();
  });

  describe("State Management", () => {
    it("should have initial state", () => {
      expect(store.userId).toBeNull();
      expect(store.firstName).toBeNull();
      expect(store.lastName).toBeNull();
      expect(store.email).toBeNull();
      expect(store.userPlan).toBeNull();
      expect(store.isLoading).toBe(false);
      expect(store.error).toBeNull();
      expect(store.auth_token).toBeNull();
    });

    it("should set user ID", () => {
      store.setUserId("test-user-id");
      expect(store.userId).toBe("test-user-id");
    });

    it("should set first name", () => {
      store.setFirstName("John");
      expect(store.firstName).toBe("John");
    });

    it("should set last name", () => {
      store.setLastName("Doe");
      expect(store.lastName).toBe("Doe");
    });

    it("should set email", () => {
      store.setEmail("john.doe@example.com");
      expect(store.email).toBe("john.doe@example.com");
    });

    it("should set and get auth token", () => {
      store.setAuthToken("test-auth-token");
      expect(store.auth_token).toBe("test-auth-token");
      expect(store.getAuthToken()).toBe("test-auth-token");
    });

    it("should set user plan", () => {
      const mockPlan = {
        id: "123",
        name: "Pro",
        features: {
          analytics: true,
          team_features: true,
          custom_domains: true,
        },
        max_pins: 10,
        created_at: "2023-01-01T00:00:00Z",
        stripe_id: "stripe_123",
      };
      store.setPlan(mockPlan);
      expect(store.userPlan).toEqual(mockPlan);
    });
  });

  describe("fetchUserData", () => {
    const mockClerkUser = {
      id: "user-id",
      email: "test@example.com",
      firstName: "Test",
      lastName: "User",
    };

    const mockApiResponse = {
      status: 200,
      data: {
        user: {
          id: "user-id",
          email: "test@example.com",
          auth_token: "test-token",
        },
        plan: {
          id: "plan-id",
          name: "Pro",
          max_pins: 6,
          features: {
            analytics: true,
            team_features: true,
            custom_domains: true,
          },
          created_at: "2023-01-01T00:00:00Z",
          stripe_id: "stripe_123",
        },
        links: [{ id: "link-1", url: "https://example.com" }],
        settings: {
          settings_blob: { theme: "dark" },
        },
      },
    };

    // it("should fetch user data successfully", async () => {
    //   vi.mocked(api.get).mockResolvedValue(mockApiResponse);

    //   const result = await store.fetchUserData(mockClerkUser);

    //   expect(result).toBe(true);
    //   expect(store.userId).toBe("user-id");
    //   expect(store.email).toBe("test@example.com");
    //   expect(store.firstName).toBe("Test");
    //   expect(store.lastName).toBe("User");
    //   expect(store.auth_token).toBe("test-token");
    //   expect(store.userPlan).toEqual(mockApiResponse.data.plan);
    //   expect(vi.mocked(cache.set)).toHaveBeenCalledWith(
    //     CacheKeys.USER,
    //     store.$state,
    //   );
    // });

    it("should handle API error", async () => {
      vi.mocked(api.get).mockRejectedValue(new Error("API error"));

      const result = await store.fetchUserData(mockClerkUser);

      expect(result).toBe(false);
      expect(store.error).toBeTruthy();
      expect(vi.mocked(cache.clear)).toHaveBeenCalledWith(CacheKeys.USER);
      expect(vi.mocked(cache.clear)).toHaveBeenCalledWith(CacheKeys.LINKS);
      expect(vi.mocked(cache.clear)).toHaveBeenCalledWith(CacheKeys.SETTINGS);
    });

    it("should use cached data if available", async () => {
      vi.mocked(cache.get).mockReturnValue(mockApiResponse);
      vi.mocked(api.get).mockResolvedValue(mockApiResponse);

      await store.fetchUserData(mockClerkUser);

      expect(vi.mocked(cache.get)).toHaveBeenCalledWith(CacheKeys.USER);
      // Check that state was first loaded from cache
      expect(store.$state).toMatchObject(
        expect.objectContaining(mockApiResponse),
      );
    });
  });

  describe("confirmSubscription", () => {
    it("should confirm subscription successfully", async () => {
      store.setUserId("user-id");
      store.setEmail("test@example.com");

      vi.mocked(api.get).mockResolvedValue({ status: 200, data: {} });

      const result = await store.confirmSubscription();

      expect(result).toBe(true);
      expect(api.get).toHaveBeenCalled();
    });

    it("should throw error if user ID is missing", async () => {
      store.setEmail("test@example.com");

      await expect(store.confirmSubscription()).rejects.toThrow(
        "User ID or email not found",
      );
    });

    it("should throw error on failed API call", async () => {
      store.setUserId("user-id");
      store.setEmail("test@example.com");

      vi.mocked(api.get).mockResolvedValue({ status: 400, data: {} });

      await expect(store.confirmSubscription()).rejects.toThrow(
        "Failed to confirm subscription",
      );
    });
  });
});
