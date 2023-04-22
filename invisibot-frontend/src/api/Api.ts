import axios, { AxiosResponse } from "axios";

axios.defaults.baseURL = "http://0.0.0.0:8000/api";

export const Api = {
  game: {
    get: () => {
      return handleResponse(axios.get("/game"));
    },
  },
};

export type Response<T> = {
  data?: T;
  success: boolean;
  error?: string;
  failedToReachBackend: boolean;
};

interface RawApiResponse<T> {
  data?: T;
  success: boolean;
  error?: string;
}

function handleResponse<T>(
  response: Promise<AxiosResponse<RawApiResponse<T>>>
): Promise<Response<T>> {
  return response
    .then((res) => {
      let resp = res.data;

      if (!resp.success) {
        return {
          success: false,
          failedToReachBackend: false,
          error: resp.error,
        };
      }

      if (res.status < 300) {
        return {
          data: resp.data,
          success: true,
          failedToReachBackend: false,
        };
      }

      return {
        success: false,
        error: "",
        failedToReachBackend: false,
      };
    })
    .catch((err) => {
      if (err.errno === -111) {
        console.error("Failed to reach backend", err);
        // Failed to reach the server
        return {
          success: false,
          failedToReachBackend: true,
          error: "Unable to reach backend",
        };
      }

      console.error("ERROR!!! ", err);

      return {
        success: false,
        failedToReachBackend: false,
        error: err.toString(),
      };
    });
}
