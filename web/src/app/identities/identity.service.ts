import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import { environment } from '../../environments/environment';

export interface Identity {
  name: string;
  publicKey: string;
}

@Injectable({
  providedIn: 'root',
})
export class IdentityService {
  constructor(private readonly http: HttpClient) {}

  public getIdentities(): Observable<Identity[]> {
    return this.http.get<Identity[]>(`${environment.backendUrl}/v1/identities`);
  }

  add(name: string): Observable<void> {
    return this.http.post<void>(`${environment.backendUrl}/v1/identities`, {
      name,
    });
  }
}
