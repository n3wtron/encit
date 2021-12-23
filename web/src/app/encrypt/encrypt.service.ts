import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { environment } from '../../environments/environment';
import { Observable } from 'rxjs';

export interface EncItMessage {
  sender: string;
  receiver: string;
  subject: string;
  messageType: string;
  payload: string;
  verified: boolean;
}

@Injectable({
  providedIn: 'root',
})
export class EncryptService {
  constructor(private readonly http: HttpClient) {}

  public encrypt(
    identity: string,
    friend: string,
    subject: string | undefined,
    messageType: 'file'|'plaintext',
    message: string
  ): Observable<string> {
    return this.http.post<string>(
      `${environment.backendUrl}/v1/encrypt`,
      {
        identity,
        friend,
        subject,
        messageType,
        message,
      },
      { responseType: 'text' as 'json' }
    );
  }

  public decrypt(message: string, identity?: string): Observable<EncItMessage> {
    return this.http.post<EncItMessage>(
      `${environment.backendUrl}/v1/decrypt`,
      {
        identity,
        message,
      }
    );
  }
}
