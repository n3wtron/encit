import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface Friend {
  name: string;
  publicKey: string;
}

@Injectable({
  providedIn: 'root',
})
export class FriendService {
  constructor(private readonly http: HttpClient) {}

  public getFriends(): Observable<Friend[]> {
    return this.http.get<Friend[]>(`${environment.backendUrl}/v1/friends`);
  }

  public add(
    name: string,
    keyFormat: 'PEM' | 'HEX' | 'BASE64',
    publicKey: string
  ): Observable<void> {
    return this.http.post<void>(`${environment.backendUrl}/v1/friends`, {
      name,
      keyFormat,
      publicKey,
    });
  }
}
