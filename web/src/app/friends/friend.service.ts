import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {Observable} from "rxjs";
import {environment} from "../../environments/environment";

export interface Friend {
  name: string,
  privateKey: string
}

@Injectable({
  providedIn: 'root'
})
export class FriendService {

  constructor(private readonly http: HttpClient) {
  }

  public getIdentities(): Observable<Friend[]> {
    return this.http.get<Friend[]>(`${environment.backendUrl}/v1/friends`);
  }
}
