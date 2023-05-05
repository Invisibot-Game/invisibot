import axios, { AxiosResponse } from "axios";
import { Round } from "./Round";

axios.defaults.baseURL = "http://localhost:3000/api";

export const Api = {
  game: {
    create: (numPlayers: number, numRounds: number) => {
      return handleResponse(
        axios.post("/game", {
          numPlayers: numPlayers,
          numRounds: numRounds,
        })
      );
    },
    get: () => {
      return handleResponse<{ rounds: Round[] }>(axios.get("/game"));
    },
    delete: () => {
      return handleResponse(axios.delete("/game"));
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
  response: Promise<AxiosResponse<T>>
): Promise<Response<T>> {
  return response
    .then((res) => {
      let resp = res.data;
      console.log("THEN", res, "\n\n\n DATA \n\n\n", resp);

      if (res.status < 300) {
        return {
          data: resp,
          success: true,
          failedToReachBackend: false,
        };
      }

      return {
        success: false,
        error: "Uncaught status code?",
        failedToReachBackend: false,
      };
    })
    .catch((err) => {
      console.log("ERR", err);
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
      let error = err as RawApiResponse<any>;

      return {
        success: false,
        failedToReachBackend: false,
        error: error.data,
      };
    });
}
